import Alpine from '@alpinejs/csp';
import htmx from 'htmx.org';
import versions from './versions.json';

// The shared package owns framework setup. Consumers only register domain data.
htmx.config.allowEval = false;
htmx.config.allowScriptTags = false;
htmx.config.includeIndicatorStyles = false;
htmx.config.selfRequestsOnly = true;
htmx.config.historyCacheSize = 0;

let started = false;
const registered = new Set();
function register(name, factory) {
  if (started) throw new Error('Register shared UI components before DOMContentLoaded');
  if (typeof name !== 'string' || !/^[A-Za-z][A-Za-z0-9_-]*$/.test(name) || typeof factory !== 'function' || registered.has(name)) {
    throw new TypeError('Invalid or duplicate shared UI component registration');
  }
  registered.add(name);
  Alpine.data(name, factory);
}
function targetElement(target) {
  const node = typeof target === 'string' ? document.querySelector(target) : target;
  if (!(node instanceof HTMLElement) || !node.matches('[data-ui-view]')) {
    throw new TypeError('Navigation target must be a shared UI view host');
  }
  return node;
}
function navigate(url, {target}) {
  const node = targetElement(target);
  const endpoint = new URL(url, location.href);
  if (endpoint.origin !== location.origin || endpoint.username || endpoint.password) {
    throw new TypeError('Shared UI navigation requires a same-origin URL');
  }
  return htmx.ajax('GET', endpoint.pathname + endpoint.search, {source: node, target: node, swap: 'innerHTML'});
}
function notify(name, detail, cancelable = false) {
  return detail.target.dispatchEvent(new CustomEvent('rusthtmxui:' + name, {detail, bubbles: true, cancelable}));
}
document.addEventListener('htmx:beforeRequest', event => {
  const {target, elt} = event.detail;
  if (!target?.matches('[data-ui-view]')) return;
  if (!notify('before-navigation', {target, source: elt}, true)) event.preventDefault();
});
document.addEventListener('htmx:afterSwap', event => {
  const {target} = event.detail;
  if (target?.matches('[data-ui-view]')) notify('navigation', {target});
});
document.addEventListener('htmx:responseError', event => {
  const {target, xhr} = event.detail;
  if (target?.matches('[data-ui-view]')) notify('navigation-error', {target, status: xhr.status});
});
window.RustHTMXUI = Object.freeze({register, navigate, versions: Object.freeze(versions)});
function start() {
  if (started) return;
  started = true;
  Alpine.start();
}
// Page/Head emit deferred scripts in order, allowing domain scripts to register
// before startup. A runtime dynamically loaded after document completion also starts.
if (document.readyState === 'complete') queueMicrotask(start);
else document.addEventListener('DOMContentLoaded', start, {once: true});
