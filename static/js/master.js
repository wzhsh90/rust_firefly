layui.use(['form', 'layer'], function () {
});

function logOff() {
    var url = "/logoff";
    postAjax(url, {}, function (res) {
        window.location.href = "/login";
    }, function () {
        layer.msg("注销发生异常");
    });
}

$(function () {
    $(".nav-menu a").click(function () {
        $(".select-nav").removeClass("select-nav");
        $(this).addClass("select-nav");
    });
    $("#logoff").on("click", function () {
        layer.confirm("确认退出？", {
            btn: ['确定', '取消'] //按钮
        }, function () {
            logOff();
        }, function () {

        });
    });
});
