import { t as E, a as l, e as S } from "../chunks/DLBGyKVC.js";
import { p as J, aa as j, f as q, a as Q, t as n, l as y, k as M, j as u, s as e, a7 as U, c as m, r as c, a8 as B } from "../chunks/CmQi0fbH.js";
import { h as V, s as p } from "../chunks/BjaYyaa_.js";
import { i as W } from "../chunks/C6bK2EJJ.js";
import { p as D } from "../chunks/B_ggA-0N.js";
import { o as X, e as Y, q as Z } from "../chunks/B21bTIl7.js";
import { B as I } from "../chunks/DPLO-ozG.js";
import { u as tt } from "../chunks/DGTOa5g8.js";
import { M as et } from "../chunks/Cf3VYlYr.js";
import { C as ot } from "../chunks/CyYIiz5-.js";
import { L as at } from "../chunks/DPJgUI14.js";
import { T as rt } from "../chunks/CLEj1OQz.js";
import { u as _ } from "../chunks/DJVZKROe.js";
import { T as st } from "../chunks/DB8eOlLu.js";
import { b as it } from "../chunks/BO1A6s0c.js";
import { u as lt } from "../chunks/Dd1y69dN.js";
var nt = E('<div class="container svelte-7qsjv3"><h1> </h1> <p> </p> <div class="btn svelte-7qsjv3"><!> <!></div> <!></div> <!> <!>', 1), ut = E("<!> <!>", 1);
function Ct(F, K) {
  J(K, true);
  let o = tt(), h = M(false), r = M(""), $ = D({ post_logout_redirect_uri: _("post_logout_redirect_uri").get(), id_token_hint: _("id_token_hint").get(), state: _("state").get() });
  j(() => {
    u(r) && Y(u(r));
  }), j(() => {
    $.id_token_hint && x();
  });
  async function k() {
    window.location.replace("/auth/v1");
  }
  async function x() {
    y(h, true);
    let t = "/auth/v1/oidc/logout";
    lt().get() && (t = "/auth/v1/dev/logout");
    let L = await it(t, $, "form");
    Z();
    let s = L.headers.get("location");
    s ? window.location.replace(s) : await k();
  }
  var T = ut();
  V((t) => {
    n(() => U.title = (o == null ? void 0 : o.logout.logout) || "Logout");
  });
  var w = q(T);
  rt(w, { id: X, get value() {
    return u(r);
  }, set value(t) {
    y(r, D(t));
  } });
  var N = e(w, 2);
  et(N, { children: (t, L) => {
    ot(t, { children: (s, mt) => {
      var P = nt(), d = q(P), f = m(d), O = m(f, true);
      c(f);
      var v = e(f, 2), R = m(v, true);
      c(v);
      var g = e(v, 2), b = m(g);
      I(b, { onclick: x, get isLoading() {
        return u(h);
      }, children: (i, H) => {
        B();
        var a = S();
        n(() => p(a, o.logout.logout)), l(i, a);
      }, $$slots: { default: true } });
      var z = e(b, 2);
      I(z, { level: 3, onclick: k, children: (i, H) => {
        B();
        var a = S();
        n(() => p(a, o.common.cancel)), l(i, a);
      }, $$slots: { default: true } }), c(g);
      var A = e(g, 2);
      W(A, (i) => {
      }), c(d);
      var C = e(d, 2);
      st(C, { absolute: true });
      var G = e(C, 2);
      at(G, { absolute: true }), n(() => {
        p(O, o.logout.logout), p(R, o.logout.confirmMsg);
      }), l(s, P);
    } });
  } }), l(F, T), Q();
}
export {
  Ct as component
};
