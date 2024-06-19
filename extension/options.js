const autoConfirmElement = document.getElementById('autoconfirm');
const overrideURIElement = document.getElementById('overrideURI');
const trayURIElement = document.getElementById('trayURI');
const submitElement = document.getElementById('submit');
const statusElement = document.getElementById('status');
const tokenElement = document.getElementById('token');

const updateURI = (e) => {
  chrome.storage.sync.set(
    {
      autoconfirm: autoConfirmElement.checked,
      overrideURI: overrideURIElement.checked,
      trayURI: trayURIElement.value,
      token: tokenElement.value
    },
    () => {
      trayURIElement.disabled = !overrideURIElement.checked;
      statusElement.textContent = 'Options saved.';
      setTimeout(() => {
        statusElement.textContent = '';
      }, 1500);
    }
  );
}

const getCurrentOptions = () => {
  chrome.storage.sync.get(
    {
      autoconfirm: false,
      overrideURI: false,
      trayURI: 'http://127.0.0.1:9221',
      token: "change_me_2"
    },
    (items) => {
      autoConfirmElement.checked = items.autoconfirm;
      overrideURIElement.checked = items.overrideURI;
      trayURIElement.value = items.trayURI;
      tokenElement.value = items.token;
      trayURIElement.disabled = !items.overrideURI;
    }
  );
}

autoConfirmElement.addEventListener('click', updateURI);
overrideURIElement.addEventListener('click', updateURI);
submitElement.addEventListener('click', updateURI);

document.addEventListener('DOMContentLoaded', getCurrentOptions);