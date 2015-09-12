mergeInto(LibraryManager.library, {
  rs_query: function (str) {
    this.rs_refs || (this.rs_refs = []);
    var value = document.querySelector(UTF8ToString(str));
    if (!value) {
        return -1;
    }
    return this.rs_refs.push(value) - 1;
  },
  rs_create: function (str) {
    this.rs_refs || (this.rs_refs = []);
    var value = document.createElement(UTF8ToString(str));
    if (!value) {
        return -1;
    }
    return this.rs_refs.push(value) - 1;
  },
  rs_append: function (id, id2) {
    this.rs_refs || (this.rs_refs = []);
    this.rs_refs[id].appendChild(this.rs_refs[id2]);
  },
  rs_html_set: function (id, str) {
    this.rs_refs || (this.rs_refs = []);
    this.rs_refs[id].innerHTML = UTF8ToString(str);
  },
  rs_html_append: function (id, str) {
    this.rs_refs || (this.rs_refs = []);
    this.rs_refs[id].insertAdjacentHTML('beforeEnd', UTF8ToString(str));
  },
  rs_html_prepend: function (id, str) {
    this.rs_refs || (this.rs_refs = []);
    this.rs_refs[id].insertAdjacentHTML('afterBegin', UTF8ToString(str));
  },
  rs_release: function (id) {
    this.rs_refs || (this.rs_refs = []);
    delete this.rs_refs[id];
  },
  rs_alert: function (str) {
    alert(UTF8ToString(str));
  },
  rs_on: function (id, type, fn, fn2) {
    this.rs_refs || (this.rs_refs = []);
    this.rs_refs[id].addEventListener(UTF8ToString(type), function () {
        Runtime.dynCall('vi', fn2, [fn]);
    }, false);
  },
});
