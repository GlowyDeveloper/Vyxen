export default async function load(path) {
  const stageEl = document.getElementById("stage");
  const statusEl = document.getElementById("status");
  const canvasEl = document.getElementById("canvas");

  function updateScale() {
    const scale = stageEl.clientWidth / 800;
    canvasEl.style.transform = `scale(${scale})`;
  }

  const ro = new ResizeObserver(updateScale);
  ro.observe(stageEl);
  updateScale();

  function setStatus(text) {
    statusEl.textContent = text;

    if (text !== " ") {
      canvasEl.style.visibility = 'hidden';
      statusEl.style.visibility = 'visible';
    } else {
      canvasEl.style.visibility = 'visible';
      statusEl.style.visibility = 'hidden';
    }
  }

  window.addEventListener("error", (e) => {
    setStatus("Crashed: " + e.message);
  });
  window.addEventListener("unhandledrejection", (e) => {
    setStatus("Crashed: " + e.reason);
  });
  
  try {
    const { default: init } = await import(path);
    await init();
    setStatus(" ");
  } catch (err) {
    console.error("WASM init failed:", err);
    setStatus("Failed to load: " + err.message);
  }
}