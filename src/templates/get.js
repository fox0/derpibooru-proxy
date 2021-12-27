(function () {
  var image_id = window.location.href.match(/\/images\/(\d+)/)[1];
  var data = localStorage.getItem(image_id);
  //todo if is null
  var o = JSON.parse(data);
  document.getElementById('image').src = o.representations.large;
  // todo
  var tags = o.tags.split(', ');
  for (var i = 0; i < tags.length; i++) {
    console.log(tags[i]);
  }
})();
