//variables
var position = undefined
var top_pad = 20

const DRONE_TURN = 30

$(document).ready(function(){
  let joystick = $("#top-circle")
  position = joystick.position();
  joystick.css({top: position.top + top_pad, left: position.left, position:'absolute'});

  joystick.draggable({
    containment: "parent",
    stop: function(event, ui){
      joystick.css({top: position.top + top_pad, left: position.left, position:'absolute'});
    },
    drag: function(event, ui){
      var rel_coords = [Math.round(position.top - event.pageY), Math.round(position.left - event.pageX)]
      invoke("move_joystick", {coords: rel_coords})
    }
  })
})
