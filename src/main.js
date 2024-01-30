//variables
var position = undefined
var top_pad = 20

const DRONE_TURN = 30

//functions
function popup_close(){
  $("#popup").hide()
}

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

function connectivity_check(){
  invoke('check_conn').then((message) => {
    switch(message){
      case "connected":
        //show "connected" popup
        if ( $('#popup').css('visibility') == 'hidden' )
          $('#popup').css('visibility','visible');
        else
          $('#popup').css('visibility','hidden')


        //change popup elements
        $("#marker-red").attr("id", "marker-green")
        $("#in-text").text("drone connected")

        break
      case "not_connected":
        //show "not connected" popup
        if ( $('#popup').css('visibility') == 'hidden' )
          $('#popup').css('visibility','visible');
        else
          $('#popup').css('visibility','hidden');

        //change popup elements
        $("#in-text").text("drone not connected")
        break
    }
  })
}

setInterval(connectivity_check, 2000)