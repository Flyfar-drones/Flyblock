var is_dragged = false

function check_overlay(div1, div2){
    var div1Offset = div1.offset();
    var div2Offset = div2.offset();

    // Check if div1 is on top of div2
    if (!(
        div1Offset.top + div1.height() <= div2Offset.top ||
        div1Offset.top >= div2Offset.top + div2.height() ||
        div1Offset.left + div1.width() <= div2Offset.left ||
        div1Offset.left >= div2Offset.left + div2.width()
    )) {
        return true
    }
    else{
        return false
    }
}

$(document).ready(function(){
    $(".block").draggable({
        scroll: false,
        stop: function (event, ui){

            var div1 = $(this)
            var div2 = $("#block-list")
            var div_panel = $(".codepanel-block")

            if (check_overlay(div1, div_panel)){
                var clone = $(this).clone().appendTo(".codepanel-block")
                clone.removeAttr("style")
                clone.draggable({
                    containment: ".codepanel-block"
                })

                $(this).removeAttr("style");
                $(this).attr("style", "position: relative;")
            }
            
            if (check_overlay(div1, div2)){
                $(this).removeAttr("style");
                $(this).attr("style", "position: relative;")
            }

            is_dragged = false
        }
    })
})