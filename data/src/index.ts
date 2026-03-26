import { rm, mkdir, readdir } from "node:fs/promises";
import { join, dirname } from "node:path";
import { spawn } from "bun";
import { tmpdir } from "node:os";
import { mkdtemp } from "node:fs/promises";
import { downloadServerJars } from "./server";
import { cleanDataDirectory, cleanReportsDirectory } from "./clean";
import { stat } from "node:fs/promises";

const SUPPORTED_VERSIONS = [
    "1.21.11",
    "1.21.9",
    "1.21.7",
    "1.21.6",
    "1.21.5",
    "1.21.4",
    "1.21.2",
    "1.21",
    "1.20.5",
    "1.20.3",
    "1.20.2",
    "1.20",
    "1.19.4",
    "1.19.3",
    "1.19.1",
    "1.19",
];

async function execute(args: string[], cwd: string) {
    const proc = spawn({
        cmd: args,
        cwd,
        stdout: "inherit",
        stderr: "inherit",
    });

    const exitCode = await proc.exited;
    if (exitCode !== 0) {
        throw new Error(`Process exited with code ${exitCode}`);
    }
}

async function exists(path: string) {
    try {
        await stat(path);
        return true;
    } catch {
        return false;
    }
}

function isAtLeast(version: string, target: string) {
    const a = version.split(".").map(Number);
    const b = target.split(".").map(Number);

    for (let i = 0; i < Math.max(a.length, b.length); i++) {
        const av = a[i] ?? 0;
        const bv = b[i] ?? 0;
        if (av > bv) return true;
        if (av < bv) return false;
    }
    return true;
}

(async () => {
    const serverJarDirectory = "servers";
    const jarFiles = await downloadServerJars(
        SUPPORTED_VERSIONS,
        serverJarDirectory,
    );

    for (const version of jarFiles) {
        const outputDirectory = join(
            process.cwd(),
            "generated",
            `V${version.version.replaceAll(".", "_")}`,
        );

        if (await exists(outputDirectory)) {
            console.log(`Skipping version ${version.version}`);
            continue;
        }

        await mkdir(outputDirectory, { recursive: true });

        const generatedDirectory = await mkdtemp(
            join(tmpdir(), `generated_${version.version}_`),
        );

        try {
            const args = [
                "java",
                "-DbundlerMainClass=net.minecraft.data.Main",
                "-jar",
                version.fileName,
                "--server",
                "--output",
                generatedDirectory,
            ];

            if (isAtLeast(version.version, "1.18")) {
                args.push("--reports");
            }

            await execute(args, serverJarDirectory);
        } catch (e) {
            console.error(`Error on ${version.version}`, e);
            await rm(outputDirectory, { recursive: true, force: true });
            continue;
        }

        console.log(`Generated ${version.version}`);

        let dataDirectory: string | null = null;
        let reportsDirectory: string | null = null;

        if (await exists(join(generatedDirectory, "data"))) {
            dataDirectory = await move(
                generatedDirectory,
                outputDirectory,
                "data",
            );
        }

        if (await exists(join(generatedDirectory, "reports"))) {
            reportsDirectory = await move(
                generatedDirectory,
                outputDirectory,
                "reports",
            );
        }

        if (dataDirectory) {
            await cleanDataDirectory(dataDirectory);
        }

        if (reportsDirectory) {
            await cleanReportsDirectory(reportsDirectory);
        }

        await rm(generatedDirectory, { recursive: true, force: true });
    }
})();

async function move(from: string, to: string, subdir: string) {
    const src = join(from, subdir);
    const dest = join(to, subdir);

    await mkdir(dest, { recursive: true });
    await copyDir(src, dest);
    return dest;
}

async function copyDir(src: string, dest: string) {
    const entries = await readdir(src, { withFileTypes: true });

    for (const entry of entries) {
        const srcPath = join(src, entry.name);
        const destPath = join(dest, entry.name);

        if (entry.isDirectory()) {
            await mkdir(destPath, { recursive: true });
            await copyDir(srcPath, destPath);
        } else if (entry.isFile()) {
            await mkdir(dirname(destPath), { recursive: true });
            await Bun.write(destPath, Bun.file(srcPath));
        }
    }
}
