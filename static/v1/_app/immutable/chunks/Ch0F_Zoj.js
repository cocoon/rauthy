import { t as p, a as v } from "./D8nUqfqE.js";
import { p as P, c as W, r as x, t as U, j as _, a9 as b, a as j } from "./D-nwkJyM.js";
import { s as k } from "./ivlJJTAR.js";
import { a as E, s as r } from "./D_HYGYLR.js";
import { p as i } from "./2rFDT0Lm.js";
import { p as h } from "./BsLhmaxX.js";
var I = p("<a><!></a>");
function w(s, e) {
  P(e, true);
  let f = i(e, "selectedStep", 3, false), g = i(e, "hideUnderline", 3, false), n = i(e, "highlight", 3, false), d = i(e, "highlightExact", 3, false), u = i(e, "highlightWithParams", 3, false), c = b(() => {
    if (f()) return "step";
    let a = h.route.id;
    if (a) {
      if (u()) {
        if (`${a}${h.url.search}`.startsWith(e.href)) return "page";
      } else if (d()) {
        if (a === e.href.split("?")[0]) return "page";
      } else if (e.highlightIncludes) {
        if (a.includes(e.highlightIncludes)) return "page";
      } else if (a && e.href.split("?")[0].endsWith(a)) return "page";
    }
  });
  var t = I();
  let l;
  var m = W(t);
  k(m, () => e.children), x(t), U(() => {
    l = E(t, 1, "font-label svelte-vdcj5m", null, l, { hideUnderline: g() }), r(t, "href", e.href), r(t, "target", e.target), r(t, "aria-current", _(c)), r(t, "data-highlight", n());
  }), v(s, t), j();
}
export {
  w as A
};
