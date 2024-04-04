function start() {
    var awesome = "";
    for(var j = 0; j < 1000000000000000; j++) {
        awesome += j.toString();
        console.log(awesome);
    }
}