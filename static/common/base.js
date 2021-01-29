var GLOBAL_BASE_PATH = "";
var LOGIN_URL = "/login";

function callBackFunction(func, data) {
    if (typeof (func) == "function") {
        if (null != data) {
            func.apply(this, [data]);
        } else {
            func.apply(this);
        }
    }
}

function constructPostReq(url, param) {
    return $.ajax({
        type: "POST",
        url: url,
        data: param
    });
}

function constructGetReq(url, param) {
    return $.ajax({
        type: "GET",
        url: url,
        data: param
    });
}

function postAjax(url, params, succ, fail) {
    params["rand"] = Math.random();
    var index = url.indexOf("/");
    var tempUrl = "";
    if (index == 0) {
        tempUrl = GLOBAL_BASE_PATH + url;
    } else {
        tempUrl = url;
    }
    $.ajax({
        type: "post",
        url: tempUrl,
        data: params,
        success: function (result) {
            if (result["code"] == 100) {
                window.location.href = LOGIN_URL;
            } else {
                callBackFunction(succ, result);
            }
        },
        error: function (xhr, errorType, error) {
            callBackFunction(fail, errorType);
        }
    });
}

function postAjaxFile(url, params, succ, fail) {
    params["rand"] = Math.random();
    var index = url.indexOf("/");
    var tempUrl = "";
    if (index == 0) {
        tempUrl = GLOBAL_BASE_PATH + url;
    } else {
        tempUrl = url;
    }
    $.ajax({
        type: "post",
        url: tempUrl,
        data: params,
        contentType: false,
        processData: false,
        success: function (result) {
            if (result["code"] == 100) {
                window.location.href = LOGIN_URL;
            } else {
                callBackFunction(succ, result);
            }
        },
        error: function (xhr, errorType, error) {
            callBackFunction(fail, errorType);
        }
    });
}

function getAjax(url, params, succ, fail) {
    params["rand"] = Math.random();
    var index = url.indexOf("/");
    var tempUrl = "";
    if (index == 0) {
        tempUrl = GLOBAL_BASE_PATH + url;
    } else {
        tempUrl = url;
    }
    $.ajax({
        type: "get",
        url: tempUrl,
        data: params,
        success: function (result) {
            if (result["code"] == 100) {
                window.location.href = LOGIN_URL;
            } else {
                callBackFunction(succ, result);
            }
        },
        error: function (xhr, errorType, error) {
            callBackFunction(fail, errorType);
        }
    });
}


function accAdd(arg1, arg2) {
    var r1, r2, m;
    if (arg1.toString().indexOf(".") != -1) {
        r1 = arg1.toString().split(".")[1].length;
    } else {
        r1 = 0;
    }
    if (arg2.toString().indexOf(".") != -1) {
        r2 = arg2.toString().split(".")[1].length;
    } else {
        r2 = 0;
    }
    m = Math.pow(10, Math.max(r1, r2));
    return (accMul(arg1, m) + accMul(arg2, m)) / m;
}

function accMul(arg1, arg2) {
    var m = 0,
        s1 = arg1.toString(),
        s2 = arg2.toString();
    if (s1.indexOf(".") != -1) {
        m += s1.split(".")[1].length
    }
    if (s2.indexOf(".") != -1) {
        m += s2.split(".")[1].length
    }
    return Number(s1.replace(".", "")) * Number(s2.replace(".", "")) / Math.pow(10, m)
}

function formatDateNew(iVal, formatStr) {
    if (iVal == 0) {
        return "";
    } else {
        return formatDate(new Date(iVal * 1000), formatStr);
    }
}

function formatDate(date, formatStr) {
    var str = formatStr;
    var Week = ['日', '一', '二', '三', '四', '五', '六'];

    str = str.replace(/yyyy|YYYY/, date.getFullYear());
    str = str.replace(/yy|YY/, (date.getYear() % 100) > 9 ? (date.getYear() % 100).toString() : '0' + (date.getYear() % 100));

    str = str.replace(/MM/, date.getMonth() >= 9 ? (date.getMonth() + 1).toString() : '0' + (date.getMonth() + 1));
    str = str.replace(/M/g, date.getMonth() + 1);

    str = str.replace(/w|W/g, Week[date.getDay()]);

    str = str.replace(/dd|DD/, date.getDate() > 9 ? date.getDate().toString() : '0' + date.getDate());
    str = str.replace(/d|D/g, date.getDate());

    str = str.replace(/hh|HH/, date.getHours() > 9 ? date.getHours().toString() : '0' + date.getHours());
    str = str.replace(/h|H/g, date.getHours());
    str = str.replace(/mm/, date.getMinutes() > 9 ? date.getMinutes().toString() : '0' + date.getMinutes());
    str = str.replace(/m/g, date.getMinutes());

    str = str.replace(/ss|SS/, date.getSeconds() > 9 ? date.getSeconds().toString() : '0' + date.getSeconds());
    str = str.replace(/s|S/g, date.getSeconds());

    return str;

}

function strDayToDate(str) {
    var parts = str.split('-');
    var mydate = new Date(parts[0], parts[1] - 1, parts[2], 0, 0, 0);
    return parseInt(mydate.getTime() / 1000, 10);
}


function isMobile() {
    var system = {
        win: false,
        mac: false,
        xll: false
    };
    var p = navigator.platform;
    system.win = p.indexOf("Win") == 0;
    system.mac = p.indexOf("Mac") == 0;
    system.x11 = (p == "X11") || (p.indexOf("Linux") == 0);
    if (system.win || system.mac || system.xll) {
        return false;
    } else {
        return true;
    }
}

function dialogArea(pc) {
    if(typeof(pc)=="string"){
        if (isMobile()) {
            pc = "96%";
        }
        return pc;
    }else{
        if (isMobile()) {
            pc[0] = "96%";
        }
        return pc;
    }

}
function layAjaxTable(table, conf) {
    if (typeof (conf["url"]) != "undefined") {
        var loading_index = layer.load(2, {shade: [.3, '#FFF']});
    }
    var defaultOp = {
        elem: '#list-table'
        , height: 'full-130'
        , method: "post"
        , limit: 20
        , cellMinWidth: 80
        , request: {
            pageName: 'page_index',
            limitName: 'page_size'
        }
        , response: {
            countName: 'total',
            dataName: 'records'
        }
        , parseData: function (res) { //res 即为原始返回的数据
            if (res["code"] == 100) {
                window.location.href =LOGIN_URL;
            } else {
                res["code"] = 0;
            }
            return res
        }
        , cols: null
        , page: { //支持传入 laypage 组件的所有参数（某些参数除外，如：jump/elem） - 详见文档
            layout: ['limit', 'count', 'prev', 'page', 'next', 'skip'] //自定义分页布局
            , curr: 1 //设定初始在第 5 页
            , groups: 1 //只显示 1 个连续页码
            , first: false //不显示首页
            , last: false //不显示尾页
        },
        done: function () {
            layer.close(loading_index);
        }
    };
    var realOpt = $.extend(defaultOp, conf);
    table.render(realOpt);
}


function strToLongDay(str) {
    var parts = str.split('-');
    var mydate = new Date(parts[0], parts[1] - 1, parts[2], 0, 0, 0);
    return parseInt(mydate.getTime() / 1000, 10);
}

function strToLongTime(str) {
    var parts = str.split(' ');
    var partsDay = parts[0].split('-');
    var partsTime = parts[1].split(':');
    var mydate = new Date(partsDay[0], partsDay[1] - 1, partsDay[2], parseInt(partsTime[0], 10), parseInt(partsTime[1], 10), parseInt(partsTime[2], 10));
    return parseInt(mydate.getTime() / 1000, 10);
}

function getCurrentTime() {
    return parseInt(new Date().getTime() / 1000);
}

function uploadFile(file, groupId, renderId) {
    var subIndex = layer.msg('附件上传中...', {
        time: 0,
        icon: 16,
        shade: 0.01
    });
    var url = "/file/upload";
    var formData = new FormData();
    formData.append("groupId", groupId);
    formData.append("file", file, file.name);
    postAjaxFile(url, formData, function (res) {
        layer.close(subIndex);
        if (res["code"] == 0) {
            var result = res["result"];
            var jsonData = {"attachName": file.name, "id": result};
            juicer.renderAppendTo("filetmpl", jsonData, renderId);
        } else {
            layer.msg(res["message"]);
        }

    }, function () {
        layer.close(subIndex);
        layer.msg("提交发生异常");
    });
}

function delFile(id, obj) {
    layer.confirm("确认删除该附件", {
        btn: ['确定', '取消'] //按钮
    }, function () {
        postAjax('/file/del', {'id': id}, function (res) {
            if (res.code == 0) {
                layer.msg(res.message);
                $(obj).parent().parent().remove();
            } else {
                layer.msg(res.message);
            }
        }, function () {
            layer.msg('提交发生异常');
        })

    }, function () {

    });
}

function formatDay(dayLng) {
    if (dayLng != 0) {
        return formatDateNew(dayLng, "yyyy-MM-dd");
    } else {
        return ""
    }
}

function regEnterQuery(fn) {
    $(document).keyup(function (event) {
        var key = window.event ? event.keyCode : event.which;
        if (key == 13 && event.target.nodeName == "INPUT") {
            fn();
        }
    });
}

function dialogGoon(msg, ok, cancel) {
    layer.open({
        type: 1,
        title: '系统提示',
        closeBtn: false,
        shade: 0.8,
        id: 'LAY_layuipro',
        btn: ['继续添加', '取消'],
        moveType: 0,
        content: "<div style='width:100%;height: 50px;line-height: 50px; text-align: center;'>" + msg + "</div>",
        yes: function (index) {
            layer.close(index);
            ok();
        },
        btn2: function (index) {
            cancel();
            closeSelfFrame();
        }
    });
}

function dialogGoonNoClose(msg, ok, cancel) {
    layer.open({
        type: 1,
        title: '系统提示',
        closeBtn: false,
        shade: 0.8,
        id: 'LAY_layuipro',
        btn: ['继续添加', '取消'],
        moveType: 0,
        content: "<div style='width:100%;height: 50px;line-height: 50px; text-align: center;'>" + msg + "</div>",
        yes: function (index) {
            layer.close(index);
            ok();
        },
        btn2: function (index) {
            cancel();
        }
    });
}

function closeSelfFrame() {
    var index = parent.layer.getFrameIndex(window.name);
    parent.layer.close(index);
}

function closeSelfFrameData(data) {
    if (parent.getChildFrameData) {
        parent.getChildFrameData(JSON.stringify(data));
    }
    closeSelfFrame();
}

function closeSelfFrameDataStr(data) {
    if (parent.getChildFrameDataStr) {
        parent.getChildFrameDataStr(data);
    }
    closeSelfFrame();
}

function closeSelfFrameData2(data) {
    if (parent.getChildFrameData2) {
        parent.getChildFrameData2(JSON.stringify(data));
    }
    closeSelfFrame();
}

function contructParamStr(param) {
    var strArr = [];
    for (var key in param) {
        strArr.push(key + '=' + param[key]);
    }
    return strArr.join("&");
}