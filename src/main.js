/*
const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}
*/
var position = undefined
var top_pad = 20

$(document).ready(() => {
  let joystick = $("#top-circle")
  position = joystick.position();
  joystick.css({top: position.top + top_pad, left: position.left, position:'absolute'});

  joystick.draggable({
    containment: "parent",
    stop: function(event, ui){
      console.log("stop event invoked")
      joystick.css({top: position.top + top_pad, left: position.left, position:'absolute'});
    }
  })
})
  /*
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
  */
