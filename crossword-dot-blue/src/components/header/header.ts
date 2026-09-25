function watchHero(header: HTMLElement, hero: HTMLElement): void {
    const observer = new IntersectionObserver(
        ([entry]) => {
            if (!entry) return;
            const gone = window.scrollY > 0 && entry.boundingClientRect.bottom <= header.offsetHeight;
            if (gone) hero.classList.add("intro-done");
            header.classList.toggle("hero-gone", gone);
        },
        { threshold: [0, 0.25, 0.5, 0.75, 1] },
    );

    observer.observe(hero);
}

const header = document.querySelector<HTMLElement>("header.top");
const hero = header?.nextElementSibling;
if (header && hero instanceof HTMLElement && hero.matches("section.hero")) watchHero(header, hero);
