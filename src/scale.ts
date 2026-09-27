import { getCurrentWebview } from "@tauri-apps/api/webview";
import { getCurrentWindow } from "@tauri-apps/api/window";

/**
 * Undo Windows "Make text bigger" (Accessibility > Text size).
 *
 * WebView2 folds that setting into the page zoom but the window size ignores
 * it, so the layout overflows its own window. Rather than reading the registry,
 * measure: logical window width / page width = the extra zoom; counter it and
 * measure again until it settles. Display scaling is handled correctly by
 * WebView2 and is left alone. Same approach as 久世管理器.
 */
export async function lockScale() {
  const win = getCurrentWindow();
  const view = getCurrentWebview();
  let zoom = 1;

  for (let i = 0; i < 3; i++) {
    await nextFrame();
    const logical = (await win.innerSize()).toLogical(await win.scaleFactor());
    const measured = window.innerWidth;
    if (!measured) return;

    const off = logical.width / measured;
    if (Math.abs(off - 1) < 0.01) return;

    zoom /= off;
    try {
      await view.setZoom(zoom);
    } catch {
      return;
    }
  }
}

function nextFrame(): Promise<void> {
  return new Promise((resolve) => requestAnimationFrame(() => resolve()));
}
