var position = undefined
var top_pad = 20

$(document).ready(function(){

  let joystick = $("#top-circle")
  position = joystick.position();
  joystick.css({top: position.top + top_pad, left: position.left, position:'absolute'});

  joystick.draggable({
    containment: "parent",
    stop: function(event, ui){
      joystick.css({top: position.top + top_pad, left: position.left, position:'absolute'});
    }
  })
})
