function App() {
  return (
    <main className="shell">
      <section className="card" aria-labelledby="title">
        <p className="eyebrow">BMDock UI</p>
        <h1 id="title">Desktop workbench</h1>
        <p className="status">Renderer ready · static shell</p>
        <p className="copy">
          This static shell does not call MCP, the filesystem, or path writes. The
          Tauri crate is the T05 host; typed IPC and the engine Supervisor belong to
          T06/T07 and are not invoked from this shell.
        </p>
      </section>
    </main>
  );
}

export default App;
