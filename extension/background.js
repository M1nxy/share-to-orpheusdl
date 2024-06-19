chrome.action.onClicked.addListener((tab) => {
  chrome.scripting.executeScript({
    target: { tabId: tab.id },
    func: main
  });
});

function main() {
  chrome.storage.sync.get(
    {
      autoconfirm: false,
      overrideURI: false,
      trayURI: 'http://127.0.0.1:9221',
      token: "change_me_2"
    },
    (items) => {
      let fetchOptions = {
        method: "post",
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${items.token}`
        },
        body: JSON.stringify({
          url: document.URL
        })
      };
      let trayURI = items.overrideURI ? items.trayURI : 'http://127.0.0.1:9221';
      if (items.autoconfirm) {
        fetch(`${trayURI}/download`, fetchOptions);
      } else {
        if (confirm("Do you want to download?")) {
          fetch(`${trayURI}/download`, fetchOptions);
        } else {
          return;
        }
      }
    }
  );
}
