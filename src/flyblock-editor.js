$(document).ready(function(){
    //do something
    $(".block").draggable({
        scroll: false,
        drag: function (event, ui) {
            //$(this).clone().appendTo(".codepanel-block");
        },
        stop: function (event, ui){
            var is_on_top = false

            var div1 = $(this)
            var div2 = $("#block-list")

            var div1Offset = div1.offset();
            var div2Offset = div2.offset();

            // Check if div1 is on top of div2
            if (!(
                div1Offset.top + div1.height() <= div2Offset.top ||
                div1Offset.top >= div2Offset.top + div2.height() ||
                div1Offset.left + div1.width() <= div2Offset.left ||
                div1Offset.left >= div2Offset.left + div2.width()
            )) {
                is_on_top = true
            }

            if (is_on_top){
                $(this).removeAttr('style');
                $(this).attr("style", "position: relative;")
            }
        }
    })
})