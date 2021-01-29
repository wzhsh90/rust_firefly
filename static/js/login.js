layui.use(['form'], function () {
});
$(document).keyup(function (event) {
    var key = window.event ? event.keyCode : event.which;
    if (key == 13) {
        login();
    }
});

function login() {
    var account = $.trim($("#account").val());
    var password = $.trim($("#password").val());
    if (account == "") {
        layer.alert("用户名不能为空");
        return;
    }
    if (password == "") {
        layer.alert("密码不能为空");
        return;
    }
    var url = "/login_api";
    postAjax(url, {
        "account": account,
        "password": password
    }, function (result) {
        if (result["code"] == 0) {
            window.location.href = result["data"];
        } else {
            layer.alert(res["msg"]);
        }

    })
}

$(function () {
    $("#login-btn").click(login);
    regEnterQuery(login);
});