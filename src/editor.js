var elem_focus = "codepanel"
var curr_line = 1
var selected_index = 0
var to_upper = false
var line_is_zero = false
var last_indent = 0

const CURSOR_JUMP_LEFT = 7
const CURSOR_JUMP_TOP = 24
const TOP_MARKER_DEFAULT = 8
const LEFT_MARKER_DEFAULT = 26

$(document).ready(function(){
    $("#code-marker").css({
        "top": `${TOP_MARKER_DEFAULT}px`,
        "left": `${LEFT_MARKER_DEFAULT}px`
    })

    //global event listener on keyboard
    $(document).keydown(function(e) {
        let key = e.key;
        let curr_value = $(`#line${curr_line} .line-content`).text()

        console.log(e.keyCode)
        switch(elem_focus){
            case "codepanel":
                if(e.keyCode == 8){ //backspace
                    var slice1 = curr_value.slice(0, -selected_index)
                    var slice2 = curr_value.slice(-selected_index)

                    if (selected_index == 0){
                        slice2 = slice2.slice(0, -1)
                    }
                    else{
                        slice1 = slice1.slice(0, -1)
                    }

                    $(`#line${curr_line} .line-content`).text(slice1 + slice2)
                    
                    if ($(`#line${curr_line} .line-content`).text().length == 0){
                        if (line_is_zero){
                            if (curr_line == 1){
                                return
                            }

                            $(`#line${curr_line}`).remove()
                            curr_line -= 1
                            line_is_zero = false
                            
                            let top_pix = parseInt($("#code-marker").css("top").slice(0, -2))
                            $("#code-marker").css({
                                "top": `${top_pix - CURSOR_JUMP_TOP}px`,
                                "left": `${last_indent}px`
                            })
                        }
                        line_is_zero = true
                    }

                    let left_pix = parseInt($("#code-marker").css("left").slice(0, -2))
                    $("#code-marker").css({"left": `${left_pix - CURSOR_JUMP_LEFT}px`})
                }
                else if(e.keyCode == 13){ //enter
                    curr_line += 1
                    $(".codepanel").append(`<p id="line${curr_line}"><span class="indent">${curr_line} </span><span class="line-content"></span></span></p>`);
                    
                    let top_pix = parseInt($("#code-marker").css("top").slice(0, -2))
                    last_indent = parseInt($("#code-marker").css("left").slice(0, -2)) + CURSOR_JUMP_LEFT

                    $("#code-marker").css({
                        "left": `${LEFT_MARKER_DEFAULT}px`,
                        "top": `${top_pix + CURSOR_JUMP_TOP}px`
                    })

                }
                else if (e.keyCode == 37){ //left arrow
                    selected_index += 1
                    $("#code-marker").css({"left": `${left_pix - CURSOR_JUMP_LEFT}px`})
                }
                else if (e.keyCode == 39){ //right arrow
                    selected_index -= 1
                    $("#code-marker").css({"left": `${left_pix + CURSOR_JUMP_LEFT}px`})
                }
                else if (e.keyCode == 16){ //shift
                    to_upper = true
                }
                else if (e.keyCode == 20){
                    if (to_upper){
                        to_upper = false
                    }
                    else{
                        to_upper = true
                    }
                }
                else{
                    if (selected_index == 0){
                        var slice1 = curr_value.slice(-selected_index)
                        var slice2 = ""
                    }
                    else{
                        var slice1 = curr_value.slice(0, -selected_index)
                        var slice2 = curr_value.slice(-selected_index)
                    }
                    
                    if (e.keyCode == 32){ //backspace
                        key = "&nbsp;"
                    }

                    let out = ""
                    if(to_upper){
                        out = (slice1 + key + slice2)
                    }
                    else{
                        out = (slice1 + key.toLowerCase() + slice2)
                    }
                    $(`#line${curr_line} .line-content`).html(out)
                    
                    let left_pix = parseInt($("#code-marker").css("left").slice(0, -2))
                    $("#code-marker").css({"left": `${left_pix + CURSOR_JUMP_LEFT}px`})
                }
            case "leftpanel":
                break
        }
    });

    $(document).keyup(function(e) {
        if (e.keyCode == 16){ //shift
            to_upper = false
        }
    })

    $(".codepanel").click(function(e) {
        elem_focus = "codepanel"
    });

    $(".leftpanel").click(function(e) {
        elem_focus = "leftpanel"
    })
})