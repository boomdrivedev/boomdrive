// The pictures of Home's benefit scenes, drawn on a canvas: things that pile,
// slide and flow, in the site's palette, real-world looking without being
// realistic. Each scene is a function of time on its own loop, so its still
// is the same drawing at one moment, and a paused scene is just not redrawn.
// The words beside a scene carry its meaning; the picture is decoration.
// Budget: this module stays under 16 KB before compression, with no library.

type Ctx = CanvasRenderingContext2D;
export type Palette = Record<"ground" | "ground2" | "brass" | "mint" | "bone" | "rust" | "ink3" | "line", string>;
export type Scene = { period: number; still: number; draw: (g: Ctx, p: Palette, t: number) => void };

// The drawing is 640 by 400; the canvas scales it.
export const W = 640, H = 400;

const clamp = (v: number) => Math.max(0, Math.min(1, v));
const seg = (t: number, a: number, b: number) => clamp((t - a) / (b - a));
const ease = (x: number) => (x < 0.5 ? 4 * x * x * x : 1 - (-2 * x + 2) ** 3 / 2);
const lerp = (a: number, b: number, x: number) => a + (b - a) * x;
const rand = (i: number) => {
  const v = Math.sin(i * 12.9898 + 78.233) * 43758.5453;
  return v - Math.floor(v);
};

// A block: a rounded tile with a soft shadow and a lit top edge.
function block(g: Ctx, x: number, y: number, w: number, h: number, fill: string, alpha = 1, tilt = 0) {
  if (alpha <= 0) return;
  g.save();
  g.globalAlpha = alpha;
  g.translate(x + w / 2, y + h / 2);
  g.rotate(tilt);
  g.shadowColor = "rgba(0,0,0,.4)";
  g.shadowBlur = 10;
  g.shadowOffsetY = 4;
  g.fillStyle = fill;
  g.beginPath();
  g.roundRect(-w / 2, -h / 2, w, h, Math.min(6, h / 3));
  g.fill();
  g.shadowColor = "transparent";
  const lit = g.createLinearGradient(0, -h / 2, 0, h / 2);
  lit.addColorStop(0, "rgba(255,255,255,.28)");
  lit.addColorStop(0.45, "rgba(255,255,255,0)");
  lit.addColorStop(1, "rgba(0,0,0,.12)");
  g.fillStyle = lit;
  g.fill();
  g.restore();
}

function label(g: Ctx, s: string, x: number, y: number, fill: string, align: CanvasTextAlign = "left", size = 13) {
  g.font = `${size}px "IBM Plex Mono", ui-monospace, monospace`;
  g.fillStyle = fill;
  g.textAlign = align;
  g.fillText(s, x, y);
}

function line(g: Ctx, pts: number[][], stroke: string, width = 1, dash: number[] = []) {
  g.save();
  g.strokeStyle = stroke;
  g.lineWidth = width;
  g.setLineDash(dash);
  g.beginPath();
  pts.forEach(([x, y], i) => (i ? g.lineTo(x, y) : g.moveTo(x, y)));
  g.stroke();
  g.restore();
}

function tick(g: Ctx, x: number, y: number, fill: string, p: number) {
  if (p <= 0) return;
  g.save();
  g.globalAlpha = p;
  g.strokeStyle = fill;
  g.lineWidth = 2.4;
  g.lineCap = "round";
  g.beginPath();
  g.moveTo(x - 6, y);
  g.lineTo(x - 2, y + 4);
  g.lineTo(x + 7, y - 6);
  g.stroke();
  g.restore();
}

// A small agent: the flywheel's bot.
function bot(g: Ctx, x: number, y: number, p: Palette, alpha = 1) {
  block(g, x - 11, y - 9, 22, 18, p.brass, alpha);
  g.save();
  g.globalAlpha = alpha;
  g.fillStyle = p.ground;
  g.beginPath();
  g.arc(x - 4, y, 2.2, 0, 7);
  g.arc(x + 4, y, 2.2, 0, 7);
  g.fill();
  g.restore();
}

// 2. No tickets to write: sources speak, an adapter captures each event
// unattended, and a session reads the captures into signals that move to the
// intents they bear on.
const capture: Scene = {
  period: 10,
  still: 9,
  draw(g, p, t) {
    const out = 1 - seg(t, 9.4, 10);
    const lanes = [100, 170, 240, 310];
    const gate = 196, pile = 282, base = 330;
    label(g, "SOURCES", 40, 52, p.ink3);
    label(g, "ADAPTERS", gate, 52, p.ink3, "center");
    label(g, "CAPTURES", pile + 26, 52, p.ink3, "center");
    label(g, "INTENTS", 600, 52, p.ink3, "right");
    // Each event is captured where it happened and kept; an adapter lights
    // as its capture passes.
    const from = [0, 1, 2, 3, 1, 0], hot = [0, 0, 0, 0];
    const cards = from.map((j, k) => {
      const s = 0.3 + k * 0.6;
      const a = ease(seg(t, s, s + 1)), b = ease(seg(t, s + 1, s + 1.7));
      hot[j] = Math.max(hot[j], Math.sin(Math.PI * seg(t, s + 0.3, s + 0.9)));
      return [lerp(lerp(136, gate + 26, a), pile, b), lerp(lanes[j] - 10, base - k * 24, b), seg(t, s, s + 0.3) * out];
    });
    ["meeting", "chat", "log", "pull request"].forEach((name, j) => {
      const y = lanes[j];
      label(g, name, 40, y + 4, p.ink3);
      line(g, [[150, y], [gate + 34, y]], p.line, 1, [3, 5]);
      g.save();
      g.strokeStyle = p.brass;
      g.fillStyle = p.brass;
      g.globalAlpha = 0.18 * hot[j];
      g.fillRect(gate - 16, y - 16, 32, 32);
      g.globalAlpha = 0.55 + 0.45 * hot[j];
      g.lineWidth = 1.5;
      g.strokeRect(gate - 16, y - 16, 32, 32);
      g.restore();
    });
    const intents = [92, 182, 272];
    for (const y of intents) {
      g.save();
      g.strokeStyle = p.brass;
      g.globalAlpha = 0.8;
      g.lineWidth = 1.5;
      g.beginPath();
      g.roundRect(462, y, 150, 56, 6);
      g.stroke();
      g.restore();
    }
    for (const [x, y, alpha] of cards) block(g, x, y, 52, 20, p.bone, alpha);
    // A session reads them into signals, and every signal moves.
    const top = base - 5 * 24;
    bot(g, pile + 26, top - 26, p, seg(t, 4.6, 5) * out);
    const to = [0, 1, 2, 0, 2, 1, 0], slot = [0, 0, 0];
    to.forEach((n, i) => {
      const s = 5.2 + i * 0.45, m = ease(seg(t, s, s + 0.8));
      const k = slot[n]++;
      if (m <= 0) return;
      const x = lerp(pile + 40, 476 + k * 26, m), y = lerp(top - 30, intents[n] + 30, m) - Math.sin(m * Math.PI) * 30;
      block(g, x, y, 18, 12, p.mint, out);
    });
  },
};

// 3. You run the fleet, not the keyboard: decisions pile up numbered, one
// word starts the batch, and the agents work it through.
const fleet: Scene = {
  period: 9,
  still: 7.2,
  draw(g, p, t) {
    const out = 1 - seg(t, 8.4, 9);
    label(g, "DECISIONS", 70, 70, p.ink3);
    label(g, "AGENTS", 300, 70, p.ink3);
    label(g, "DONE", 600, 70, p.ink3, "right");
    const lanes = [130, 210, 290];
    for (const y of lanes) line(g, [[330, y], [600, y]], p.line, 1, [3, 5]);
    // Three numbered decisions fall onto the pile.
    for (let i = 0; i < 3; i++) {
      const a = ease(seg(t, 0.3 + i * 0.5, 0.9 + i * 0.5));
      const lift = ease(seg(t, 3 + i * 0.25, 3.8 + i * 0.25));
      const x = lerp(64, 290, lift), y = lerp(lerp(-50, 220 - i * 16, a), lanes[i] - 9, lift);
      const fade = 1 - seg(t, 3.7 + i * 0.25, 4 + i * 0.25);
      block(g, x, y, 96, 34, p.bone, Math.min(a, fade) * out, lerp((i - 1) * 0.05, 0, lift));
      if (a > 0 && fade > 0) {
        g.save();
        g.globalAlpha = Math.min(a, fade) * out;
        label(g, `${i + 1}`, x + 14, y + 23, p.ground, "left", 15);
        g.globalAlpha *= 0.35;
        g.fillStyle = p.ground;
        g.fillRect(x + 32, y + 13, 46, 3);
        g.fillRect(x + 32, y + 20, 32, 3);
        g.restore();
      }
    }
    // The next decisions are already piling up while the agents work.
    for (let i = 0; i < 2; i++) {
      const a = ease(seg(t, 5.8 + i * 0.6, 6.4 + i * 0.6));
      const y = lerp(-50, 220 - i * 16, a);
      block(g, 64, y, 96, 34, p.bone, a * out, (i - 0.5) * 0.06);
      if (a > 0) {
        g.save();
        g.globalAlpha = a * out;
        label(g, `${i + 4}`, 78, y + 23, p.ground, "left", 15);
        g.restore();
      }
    }
    // One word starts the batch.
    const go = seg(t, 2.1, 2.5) * (1 - seg(t, 3.4, 3.8)) * out;
    if (go > 0) {
      g.save();
      g.globalAlpha = go;
      block(g, 84, 156, 56, 26, p.brass);
      label(g, "go", 112, 174, p.ground, "center", 14);
      g.restore();
    }
    // The agents take the work and their units flow to the done pile.
    lanes.forEach((y, j) => {
      const on = seg(t, 3.6 + j * 0.25, 4 + j * 0.25) * out;
      bot(g, 310, y, p, on);
      for (let k = 0; k < 4; k++) {
        const s = 4.2 + j * 0.3 + k * 1;
        const m = ease(seg(t, s, s + 0.9));
        if (m <= 0) continue;
        block(g, lerp(330, 584 - k * 24, m), y - 8, 20, 16, p.mint, out);
      }
    });
  },
};

// 4. Every day the same: four places, the same check at the same gate, the
// same answer.
const same: Scene = {
  period: 7,
  still: 3.6,
  draw(g, p, t) {
    const places = ["worktree", "laptop", "CI", "release"];
    const gate = 360;
    label(g, "THE SAME GATE", gate, 52, p.ink3, "center");
    places.forEach((name, j) => {
      const y = 92 + j * 78;
      label(g, name, 40, y + 4, p.ink3);
      line(g, [[130, y], [600, y]], p.line, 1, [3, 5]);
      g.save();
      g.strokeStyle = p.brass;
      g.globalAlpha = 0.55;
      g.lineWidth = 1.5;
      g.strokeRect(gate - 26, y - 22, 52, 44);
      g.restore();
      const d = j * 0.3;
      const x = lerp(120, 580, ease(seg(t, 0.4 + d, 5.6 + d)));
      const alpha = seg(t, 0.4 + d, 0.8 + d) * (1 - seg(t, 5.8 + d, 6.4 + d));
      block(g, x - 22, y - 12, 44, 24, p.bone, alpha);
      if (x > gate) tick(g, gate + 44, y - 26, p.mint, alpha * seg(x, gate, gate + 40));
    });
  },
};

// 5. Proof you can trust: work is checked before it reaches you; a failure
// goes back; work already proved passes without being run again.
const proof: Scene = {
  period: 10,
  still: 7.4,
  draw(g, p, t) {
    const belt = 230, gx = 300, gw = 84;
    line(g, [[30, belt + 14], [610, belt + 14]], p.line, 1);
    g.save();
    g.strokeStyle = p.brass;
    g.lineWidth = 1.5;
    g.strokeRect(gx, belt - 34, gw, 60);
    g.restore();
    label(g, "VERIFY", gx + gw / 2, belt - 44, p.ink3, "center");
    const arc = Array.from({ length: 25 }, (_, i) => [lerp(gx - 24, gx + gw + 24, i / 24), belt - 12 - Math.sin((i / 24) * Math.PI) * 96]);
    line(g, arc, p.mint, 1, [4, 5]);
    label(g, "ALREADY PROVED", gx + gw / 2, belt - 118, p.ink3, "center");
    label(g, "TO YOU", 584, belt + 40, p.ink3, "center");
    g.save();
    g.fillStyle = p.ground2;
    g.fillRect(552, belt + 14, 64, 6);
    g.restore();
    const out = 1 - seg(t, 9.4, 10);
    const kinds = ["pass", "fail", "proved", "pass", "pass"] as const;
    let piled = 0;
    kinds.forEach((kind, i) => {
      const s = 0.2 + i * 1.6;
      let alpha = seg(t, s, s + 0.3) * out;
      let x = lerp(10, kind === "proved" ? gx - 40 : gx + 20, ease(seg(t, s, s + 1.2)));
      let y = belt - 12;
      let fill = p.bone;
      if (kind === "proved") {
        // Over the gate on the dashed path, never into it.
        fill = p.mint;
        const u = ease(seg(t, s + 1.2, s + 2.2));
        if (u > 0) {
          x = lerp(gx - 40, gx + gw + 16, u);
          y -= Math.sin(u * Math.PI) * 96;
        }
      } else if (t > s + 1.5) fill = kind === "pass" ? p.mint : p.rust;
      if (kind === "fail") {
        const back = ease(seg(t, s + 2.1, s + 3.4));
        y += back * 70;
        x -= back * 240;
        alpha *= 1 - seg(t, s + 3, s + 3.4);
      } else {
        const leave = kind === "proved" ? seg(t, s + 2.3, s + 3.1) : seg(t, s + 2, s + 3);
        if (leave > 0) x = lerp(kind === "proved" ? gx + gw + 16 : gx + 20, 562, ease(leave));
        if (leave >= 1) y = belt - 12 - piled++ * 18;
      }
      block(g, x, y, 44, 24, fill, alpha);
    });
  },
};

// 6. Output scales with input: the journey climbing, one agent toward a
// thousand on a log scale, the field filling as it goes.
const climb: Scene = {
  period: 10,
  still: 7.6,
  draw(g, p, t) {
    const x0 = 96, x1 = 600, yAt = (n: number) => 340 - (Math.log10(n) / 3) * 270;
    [1, 10, 100, 1000].forEach((n) => {
      line(g, [[x0, yAt(n)], [x1, yAt(n)]], p.line, 1, [2, 6]);
      label(g, n.toLocaleString("en"), x0 - 14, yAt(n) + 4, p.ink3, "right");
    });
    label(g, "AGENTS", x0 - 14, 50, p.ink3, "right");
    const pr = ease(seg(t, 0.4, 7)) * (1 - seg(t, 9.4, 10));
    const curve = (u: number) => [lerp(x0, x1, u), yAt(10 ** (3 * u ** 1.15))];
    // The field under the line: as many dots as agents, near enough.
    const count = Math.round(10 ** (3 * pr));
    for (let i = 0; i < Math.min(count, 700); i++) {
      const u = rand(i) * pr, [cx, cy] = curve(u);
      const yy = lerp(cy, 340, rand(i + 999) ** 1.6);
      g.fillStyle = i % 9 ? p.mint : p.brass;
      g.globalAlpha = 0.7;
      g.fillRect(cx - 1.6, yy - 1.6, 3.2, 3.2);
      g.globalAlpha = 1;
    }
    const pts: number[][] = [];
    for (let u = 0; u <= pr; u += 0.01) pts.push(curve(u));
    line(g, pts, p.brass, 2.5);
    const [hx, hy] = curve(pr);
    g.save();
    g.fillStyle = p.brass;
    g.shadowColor = p.brass;
    g.shadowBlur = 18;
    g.beginPath();
    g.arc(hx, hy, 6, 0, 7);
    g.fill();
    g.restore();
  },
};

// 7. Built for teams that ship: bolts land on main in a steady stream and
// leave as releases, one after another.
const ship: Scene = {
  period: 9,
  still: 6.8,
  draw(g, p, t) {
    const main = 210, end = 500;
    line(g, [[40, main], [end, main]], p.mint, 2);
    label(g, "MAIN", 40, main - 14, p.ink3);
    label(g, "RELEASES", 572, 382, p.ink3, "center");
    const out = 1 - seg(t, 8.4, 9);
    let shipped = 0;
    for (let k = 0; k < 8; k++) {
      const s = 0.2 + k * 0.85;
      const from = k % 2 ? 330 : 90, x0 = 90 + (k % 4) * 70;
      const m = ease(seg(t, s, s + 0.8));
      const r = seg(t, s + 0.8, s + 0.8 + (end - x0 - 40) / 160);
      let x = lerp(x0, x0 + 40, m), y = lerp(from, main, m) - 10;
      if (r > 0) x = lerp(x0 + 40, end, r);
      if (m > 0 && r < 1) {
        line(g, [[x0, from], [x0 + 40, main]], p.line, 1, [3, 4]);
        block(g, x - 14, y, 28, 20, p.bone, out);
      }
      if (r >= 1) shipped++;
    }
    // Every few bolts that reach the end leave together as a release.
    const crates = Math.floor(shipped / 2);
    for (let c = 0; c < crates; c++) {
      const y = 330 - c * 34;
      block(g, 540, y, 64, 28, p.bone, out);
      g.save();
      g.globalAlpha = out;
      g.fillStyle = p.brass;
      g.fillRect(540, y + 11, 64, 5);
      g.restore();
    }
    line(g, [[end, main - 16], [end, main + 16]], p.brass, 2);
  },
};

export const scenes: Record<string, Scene> = { capture, fleet, same, proof, climb, ship };
