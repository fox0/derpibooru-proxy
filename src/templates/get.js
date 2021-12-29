function _parse() {
  var result = '';
  var image_id = window.location.href.match(/\/images\/(\d+)/)[1];
  var data = localStorage.getItem(image_id);
  if (!data) {
    return '#' + image_id + ' not found in localStorage';
  }
  var o = JSON.parse(data);
  //console.log(o);

  result += '<div class="header">'
    + '<span>' + o.faves + ' / [' + o.upvotes + ' ' + o.score + ' ' + o.downvotes + ']</span>'
    + '<a href="' + o.representations.full + '">[full]</a>'
    + '</div>';

  if (o.mime_type === 'video/webm') {
    result += '<video src="' + o.representations.large + '" autoplay muted controls></video>';
  } else {
    result += '<img src="' + o.representations.large + '"/>';
  }

  result += '<div class="tags">';
  var tags = o.tags.split(', ');
  for (var i = 0; i < tags.length; i++) {
    result += '<a href="/search?q=' + tags[i] + '">' + tags[i] + '</a>';
  }
  result += '</div>';

  return result;
}
document.getElementById('image').innerHTML = _parse();
