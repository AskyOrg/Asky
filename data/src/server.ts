import { join } from "path";
import { mkdir } from "node:fs/promises";

type VersionManifest = {
	versions: {
		id: string;
		url: string;
	}[];
};

type Version = {
	downloads: {
		server: {
			url: string;
			sha1: string;
			size: number;
		};
	};
};

const HOSTS = {
	versionManifests:
		"https://launchermeta.mojang.com/mc/game/version_manifest.json",
};

const api = async <T>(url: string): Promise<T> => {
	const res = await fetch(url);
	return res.json() as Promise<T>;
};

type Progress = {
	total: number;
	loaded: number;
	start: number;
};

const progresses = new Map<string, Progress>();
let lines: string[] = [];
let lastRender = 0;

function formatLine(name: string, p: Progress) {
	const percent = p.total ? ((p.loaded / p.total) * 100).toFixed(1) : "??";

	const barLength = 20;
	const filled = p.total ? Math.round((p.loaded / p.total) * barLength) : 0;

	const bar = "█".repeat(filled) + " ".repeat(barLength - filled);

	const elapsed = (Date.now() - p.start) / 1000;
	const speed = p.loaded / (elapsed || 1);
	const speedMB = (speed / 1024 / 1024).toFixed(2);

	return `${name.padEnd(12)} [${bar}] ${percent}% ${speedMB} MB/s`;
}

function render() {
	const now = Date.now();
	if (now - lastRender < 16) return;
	lastRender = now;

	const newLines = Array.from(progresses.entries()).map(([name, p]) =>
		formatLine(name, p),
	);

	if (lines.length === 0) {
		console.log(newLines.join("\n"));
		lines = newLines;
		return;
	}

	process.stdout.write(`\x1b[${lines.length}A`);

	for (let i = 0; i < newLines.length; i++) {
		if (lines[i] !== newLines[i]) {
			process.stdout.write("\x1b[2K");
			process.stdout.write(newLines[i] as any);
		}
		process.stdout.write("\n");
	}

	lines = newLines;
}

async function downloadJar(
	versionName: string,
	versionUrl: string,
	savePath: string,
) {
	const version = await api<Version>(versionUrl);

	const res = await fetch(version.downloads.server.url);

	const total = Number(res.headers.get("content-length") ?? 0);

	progresses.set(versionName, {
		total,
		loaded: 0,
		start: Date.now(),
	});

	const writer = Bun.file(savePath).writer();
	const reader = res.body!.getReader();

	while (true) {
		const { done, value } = await reader.read();
		if (done) break;

		progresses.get(versionName)!.loaded += value.length;

		await writer.write(value);

		render();
	}

	await writer.end();

	progresses.delete(versionName);
	render();
}

type ServerJar = {
	version: string;
	path: string;
	fileName: string;
	exists: boolean;
};

export async function downloadServerJars(
	versionsToDownload: string[],
	savePath: string,
): Promise<ServerJar[]> {
	await mkdir(savePath, { recursive: true });

	const checks = await Promise.all(
		versionsToDownload.map(async (version) => {
			const fileName = `${version}.jar`;
			const path = join(savePath, fileName);
			const exists = await Bun.file(path).exists();
			return { version, fileName, path, exists };
		}),
	);

	if (checks.every((e) => e.exists)) return checks;

	const manifest = await api<VersionManifest>(HOSTS.versionManifests);

	const wanted = new Set(versionsToDownload);

	const targets = manifest.versions.filter((v) => wanted.has(v.id));

	console.log(`Downloading ${targets.length} versions...\n`);

	const limit = 4;

	for (let i = 0; i < targets.length; i += limit) {
		await Promise.all(
			targets.slice(i, i + limit).map(async (v) => {
				const path = join(savePath, `${v.id}.jar`);

				if (!(await Bun.file(path).exists())) {
					await downloadJar(v.id, v.url, path);
				}
			}),
		);
	}

	console.log("\nAll downloads completed");

	return checks.map((c) => ({ ...c, exists: true }));
}
