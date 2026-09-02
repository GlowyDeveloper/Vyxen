document.addEventListener("DOMContentLoaded", () => {
    const sidebar = document.querySelector(".chapter");

    if (!sidebar) return;

    const children = sidebar.children.length;

    const items = [sidebar.children[children-3], sidebar.children[children-2], sidebar.children[children-1]];

    items.forEach(item => {
        const span = item.children[0];
        const a = span.children[0];
        const strong = a.children[0];
        const href = a.getAttribute("href");
     
        if (href === "placeholders/1.html") {
          a.href = "https://github.com/GlowyDeveloper/Vyxen";
        } else if (href === "placeholders/2.html") {
          a.href = "https://github.com/GlowyDeveloper/Vyxen/issues";
        } else if (href === "placeholders/3.html") {
          a.href = "https://docs.rs/vyxen";
        }
      
        strong.remove();
    });
});