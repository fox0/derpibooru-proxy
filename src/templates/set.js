(function () {
  var items = document.getElementsByClassName('thumb');
  for (var i = 0; i < items.length; i++) {
    var dataset = items[i].dataset;
    localStorage.setItem(dataset.id, dataset.json);
    //var data = {'key1':'value1','key2':'value2','key3':'value3'};
    //localStorage.setItem(dataset.id, JSON.stringify(data));
  }
})();
