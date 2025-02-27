const E = "modulepreload", y = function(a, i) {
  return new URL(a, i).href;
}, m = {}, w = function(i, l, u) {
  let f = Promise.resolve();
  if (l && l.length > 0) {
    const r = document.getElementsByTagName("link"), e = document.querySelector("meta[property=csp-nonce]"), h = (e == null ? void 0 : e.nonce) || (e == null ? void 0 : e.getAttribute("nonce"));
    f = Promise.allSettled(l.map((t) => {
      if (t = y(t, u), t in m) return;
      m[t] = true;
      const o = t.endsWith(".css"), v = o ? '[rel="stylesheet"]' : "";
      if (!!u) for (let s = r.length - 1; s >= 0; s--) {
        const c = r[s];
        if (c.href === t && (!o || c.rel === "stylesheet")) return;
      }
      else if (document.querySelector(`link[href="${t}"]${v}`)) return;
      const n = document.createElement("link");
      if (n.rel = o ? "stylesheet" : E, o || (n.as = "script"), n.crossOrigin = "", n.href = t, h && n.setAttribute("nonce", h), document.head.appendChild(n), o) return new Promise((s, c) => {
        n.addEventListener("load", s), n.addEventListener("error", () => c(new Error(`Unable to preload CSS for ${t}`)));
      });
    }));
  }
  function d(r) {
    const e = new Event("vite:preloadError", { cancelable: true });
    if (e.payload = r, window.dispatchEvent(e), !e.defaultPrevented) throw r;
  }
  return f.then((r) => {
    for (const e of r || []) e.status === "rejected" && d(e.reason);
    return i().catch(d);
  });
};
export {
  w as _
};
