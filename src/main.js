const { invoke } = window.__TAURI__.tauri;

let valorEncriptado;
let resultado;
let ambienteUat;
let ambienteProd;

async function greet() {
  resultado.value = await invoke("desencriptar", { valorEncriptado: valorEncriptado.value ,
    ambienteUat: ambienteUat.checked,
    ambienteProd: ambienteProd.checked});
}

window.addEventListener("DOMContentLoaded", () => {
  ambienteUat = document.querySelector("#r-uat");
  ambienteProd = document.querySelector("#r-prod");
  valorEncriptado = document.querySelector("#valor-encriptado");
  resultado = document.querySelector("#resultado");
  console.log(ambienteUat);
  console.log(ambienteProd);
  document.querySelector("#form-polaris").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
