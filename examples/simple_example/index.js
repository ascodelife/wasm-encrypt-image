import "./font/arial.ttf";
import { encrypt_image } from "wasm_encrypt_image";
const defaultInfo = [
  {
    cipher: "\u{1d}\u{14}\u{14}",
    position: { x: 30, y: 30 },
    font_style: { size: 16 },
  },
  {
    cipher: "\u{19}\u{1a}\t",
    position: { x: 50, y: 50 },
    font_style: { size: 30 },
  },
];
let p = document.createElement("p");
p.style.setProperty("white-space", "break-spaces");
p.innerText = `原始数据为：${JSON.stringify(defaultInfo, null, "    ")}`;
document.body.appendChild(p);

console.log("load wasm");
encrypt_image({ render_info: defaultInfo, user_token: "123" });
