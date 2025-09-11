// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="philosophy.html"><strong aria-hidden="true">1.</strong> Philosophy</a></li><li class="chapter-item expanded "><a href="licence.html"><strong aria-hidden="true">2.</strong> Licence</a></li><li class="chapter-item expanded "><a href="contact.html"><strong aria-hidden="true">3.</strong> Contacts</a></li><li class="chapter-item expanded affix "><li class="part-title">I - Tutorials</li><li class="chapter-item expanded "><a href="chapter_1/1-get-started.html"><strong aria-hidden="true">4.</strong> Get Started</a></li><li class="chapter-item expanded "><a href="chapter_1/2-installation.html"><strong aria-hidden="true">5.</strong> Installation</a></li><li class="chapter-item expanded "><a href="chapter_1/3-contribute.html"><strong aria-hidden="true">6.</strong> Contribute</a></li><li class="chapter-item expanded "><a href="chapter_1/4-usage.html"><strong aria-hidden="true">7.</strong> Global Usages</a></li><li class="chapter-item expanded "><a href="chapter_1/usages/1-Interacting-with-blockchain-with-nodejs.html"><strong aria-hidden="true">8.</strong> Node Connection</a></li><li class="chapter-item expanded affix "><li class="part-title">II - How To</li><li class="chapter-item expanded affix "><li class="part-title">1. Cryptanalysis for Reverse engineering and malware analysis</li><li class="chapter-item expanded "><a href="chapter_2/Reversing/1-caesar_shellcode_cryptanalysis_attack.html"><strong aria-hidden="true">9.</strong> IC against A Caesar Encryption Shellcode With r2pipe</a></li><li class="chapter-item expanded affix "><li class="part-title">2. Blockchain and cryptocurrencies cryptanalysis</li><li class="chapter-item expanded "><a href="chapter_2/Blockchain/1-Evaluating-bitcoin-wallet-collision-probability.html"><strong aria-hidden="true">10.</strong> Birthday Paradox On Ethereum Wallets.</a></li><li class="chapter-item expanded affix "><li class="part-title">3. Virology Research: Birthday Pardadox to study how much percent a yara rule could have false positive</li><li class="chapter-item expanded "><a href="chapter_2/Reversing/2-yara_rule_false_positive_percentile.html"><strong aria-hidden="true">11.</strong> Birthday Paradox On yara rule: detecting false positive percent.</a></li><li class="chapter-item expanded affix "><li class="part-title">III - API Ref</li><li class="chapter-item expanded "><a href="chapter_3/api-ref.html"><strong aria-hidden="true">12.</strong> API-Reference</a></li><li class="chapter-item expanded affix "><li class="part-title">IV - Explanations</li><li class="chapter-item expanded "><a href="chapter_4/1-goals-of-cryptanalysis.html"><strong aria-hidden="true">13.</strong> Why doing Cryptanalysis</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
