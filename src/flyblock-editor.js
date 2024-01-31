$(document).ready(function(){
    //do something
    $(".block").draggable({
        scroll: false,
        drag: function (event, ui) {
            console.log("Dragging...");
        },
        stop: function (event, ui){
            console.log($(this).parents("#parentDiv").length)
            if ($(this).parents("#block-list").length > 0){
                $(this).removeAttr('style');
                $(this).attr("style", "position: relative;")
            }
        }
    })
})