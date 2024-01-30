var elem_focus = "codepanel"
var curr_line = 1

$(document).ready(function(){
    //global event listener on keyboard
    $(document).keypress(function(e) {
        let key = String.fromCharCode(e.keyCode);
        let curr_value = $(`#line${curr_line} .line-content`).text()

        switch(elem_focus){
            case "codepanel":
                console.log(e.keyCode)
                if(e.keyCode == 8){
                    $(`#line${curr_line} .line-content`).text(curr_value.slice(0, -1))
                }
                else if(e.keyCode == 13){
                    curr_line += 1
                    $(".codepanel").append(`<p id="line${curr_line}"><span class="indent">${curr_line} </span><span class="line-content"></span></span></p>`);
                }
                else{
                    $(`#line${curr_line} .line-content`).text(curr_value + key)
                }
            case "leftpanel":
                break
        }
    });
    $(".codepanel").click(function(e) {
        elem_focus = "codepanel"
    });

    $(".leftpanel").click(function(e) {
        elem_focus = "leftpanel"
    })
})