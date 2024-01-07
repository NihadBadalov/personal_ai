const { argv } = require('node:process');
const { JSDOM } = require('jsdom');

async function main() {
  if (argv.length < 3) return console.log('No URL specified');
  if (argv.length > 3) return console.log('Too many arguments');

  const query = argv[2].replaceAll('\\', '');

  try {
    const response = await fetch(`https://html.duckduckgo.com/html/?q=${encodeURIComponent(query.toLowerCase())}`, {
      headers: {
        accept: 'text/html',
      },
    });

    const html = await response.text();

    /** @type {JSDOM} */
    const dom = new JSDOM(html);

    const links = dom.window.document.body.querySelectorAll('.web-result > .links_main > a.result__snippet');

    let linkContext = '';

    links.forEach((link) => {
      linkContext += link.textContent;
    });

    console.log(linkContext);
  } catch (e) {
    console.error(e?.message);
  }
}
main();
