import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";

// Compile with the installed compiler; no download, loader or test framework.
const root = dirname(dirname(fileURLToPath(import.meta.url)));
const cache = join(root, "node_modules", ".cache");
mkdirSync(cache, { recursive: true });
const output = mkdtempSync(join(cache, "bmdock-behavior-"));
try {
  const program = ts.createProgram([join(root, "tests", "shell-fixtures.ts"), join(root, "tests", "interaction-fixtures.ts")], {
    target: ts.ScriptTarget.ES2022,
    module: ts.ModuleKind.CommonJS,
    moduleResolution: ts.ModuleResolutionKind.Node10,
    strict: true,
    jsx: ts.JsxEmit.ReactJSX,
    esModuleInterop: true,
    skipLibCheck: true,
    rootDir: root,
    outDir: output,
    noEmitOnError: true,
  });
  const diagnostics = ts.getPreEmitDiagnostics(program);
  if (diagnostics.length) {
    console.error(ts.formatDiagnosticsWithColorAndContext(diagnostics, {
      getCanonicalFileName: (name) => name,
      getCurrentDirectory: () => root,
      getNewLine: () => "\n",
    }));
    process.exitCode = 1;
  } else {
    const emitted = program.emit();
    if (emitted.emitSkipped) throw new Error("Behavior check compilation emitted no files");
    writeFileSync(join(output, "package.json"), '{"type":"commonjs"}\n');
    const result = spawnSync(process.execPath, ["--test", "--test-isolation=none", join(root, "tests", "shell.test.mjs"), join(root, "tests", "interaction.test.mjs")], {
      stdio: "inherit",
      env: { ...process.env, BMDOCK_BEHAVIOR_OUTPUT: output },
    });
    if (result.error) throw result.error;
    process.exitCode = result.status ?? 1;
  }
} finally {
  // output is the exact unique directory created above inside this package.
  rmSync(output, { recursive: true, force: true });
}
