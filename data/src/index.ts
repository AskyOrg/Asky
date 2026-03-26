import { rm, mkdir, readdir } from "node:fs/promises";
import { join, dirname } from "node:path";
import { spawn } from "bun";
import { tmpdir } from "node:os";
import { mkdtemp } from "node:fs/promises";
import { access, constants } from "node:fs";
import { downloadServerJars } from "./server";

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
	"1.18.2",
	"1.18",
	"1.17.1",
	"1.17",
	"1.16.4",
	"1.16.3",
	"1.16.2",
	"1.16.1",
	"1.16",
	"1.15",
	"1.14",
	"1.13",
	"1.12.1",
	"1.12",
	"1.11",
	"1.10",
	"1.9.3",
	"1.9",
	"1.8",
	"1.7.2",
	"1.7",
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
		await access(path, constants.F_OK, () => {});
		return true;
	} catch {
		return false;
	}
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

		const generatedDirectory = await mkdtemp(
			join(tmpdir(), `generated_${version.version}_`),
		);

		try {
			// Certaines versions ne supportent pas --reports
			const args = [
				"java",
				"-DbundlerMainClass=net.minecraft.data.Main",
				"-jar",
				join(serverJarDirectory, version.fileName),
				"--server",
				"--output",
				generatedDirectory,
			];

			if (parseFloat(version.version) >= 1.18) {
				args.splice(5, 0, "--reports"); // insert --reports après le jar
			}

			await execute(args, serverJarDirectory);
		} catch (e) {
			console.error(`Error on ${version.version}`, e);
			continue;
		}

		console.log(`Generated ${version.version}`);

		// Vérifie si les dossiers existent avant de les copier
		await Promise.all([
			(await exists(join(generatedDirectory, "data")))
				? move(generatedDirectory, outputDirectory, "data")
				: null,
			(await exists(join(generatedDirectory, "reports")))
				? move(generatedDirectory, outputDirectory, "reports")
				: null,
		]);

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
