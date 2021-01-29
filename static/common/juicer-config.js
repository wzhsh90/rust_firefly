juicer.set({
    'tag::operationOpen': '{{',
    'tag::operationClose': '}}',
    'tag::interpolateOpen': '@{',
    'tag::interpolateClose': '}',
    'tag::noneencodeOpen': '@@{',
    'tag::noneencodeClose': '}',
    'tag::commentOpen': '{#',
    'tag::commentClose': '}'
});
function _trim(str) {
    str = str.replace(/^\s\s*/, ''),
        ws = /\s/,
        i = str.length;
    while (ws.test(str.charAt(--i)));
    return str.slice(0, i + 1);
}
juicer.renderTo = function (tplId, data, renderId) {
    var template = _trim($("#" + tplId).html());
    var html = juicer(template, data);
    $("#" + renderId).html(html);
};
juicer.renderAppendTo = function (tplId, data, renderId) {
    var template = _trim($("#" + tplId).html());
    var html = juicer(template, data);
    $("#" + renderId).append(html);
};
juicer.renderHtml = function (tplId, data) {
    var template = _trim($("#" + tplId).html());
    var html = juicer(template, data);
    return html;
};