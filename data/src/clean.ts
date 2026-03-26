import { opendir, rm } from "node:fs/promises";
import { join, posix } from "node:path";

function normalize(p: string) {
    return p.split("\\").join("/");
}

async function cleanDirectory(
    directory: string,
    relPath: string,
    toKeep: string[],
): Promise<void> {
    let dir;

    try {
        dir = await opendir(directory);
    } catch (e: any) {
        if (e.code === "ENOENT") return;
        throw e;
    }

    try {
        for await (const entry of dir) {
            const entryRelPath = relPath
                ? normalize(join(relPath, entry.name))
                : entry.name;

            const fullPath = join(directory, entry.name);

            if (entry.isDirectory()) {
                const isExactKeep = toKeep.includes(entryRelPath);

                if (isExactKeep) continue;

                const isParentOfKeep = toKeep.some((allowed) =>
                    normalize(allowed).startsWith(entryRelPath + "/"),
                );

                if (isParentOfKeep) {
                    await cleanDirectory(fullPath, entryRelPath, toKeep);
                } else {
                    await rm(fullPath, { recursive: true, force: true });
                }
            } else {
                const shouldKeepFile = toKeep.some((allowed) =>
                    entryRelPath.startsWith(normalize(allowed) + "/"),
                );

                if (!shouldKeepFile) {
                    await rm(fullPath, { force: true });
                }
            }
        }
    } finally {
        await dir.close();
    }
}

export async function cleanDataDirectory(
    path: string,
    toKeep: string[] = REGISTRIES_TO_SEND,
): Promise<void> {
    const minecraftDir = join(path, "minecraft");
    await cleanDirectory(minecraftDir, "", toKeep.map(normalize));
}

const REGISTRIES_TO_SEND = [
    "damage_type",
    "dimension_type",
    "painting_variant",
    "wolf_variant",
    "worldgen/biome",

    // 1.21.5
    "cat_variant",
    "chicken_variant",
    "cow_variant",
    "frog_variant",
    "pig_variant",
    "wolf_sound_variant",

    // 1.21.6
    "dialog",
    "tags/dialog",

    // 1.21.11
    "zombie_nautilus_variant",
    "timeline",
    "tags/timeline",
];

export async function cleanReportsDirectory(path: string): Promise<void> {
    // Only keep the packets.json file
    const dir = await opendir(path);
    for await (const dirent of dir) {
        if (
            dirent.name !== "packets.json" &&
            dirent.name !== "blocks.json" &&
            dirent.name !== "registries.json"
        ) {
            const direntPath = join(path, dirent.name);
            await rm(direntPath, { recursive: true, force: true });
        }
    }
}
