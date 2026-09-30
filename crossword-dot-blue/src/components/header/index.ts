import type { HeroWatch } from "@setups";

export const heroWatch: HeroWatch = (header, { signal }) => {
    const hero = header.nextElementSibling;
    if (!(hero instanceof HTMLElement) || !hero.matches("section.hero")) return;

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
    signal.addEventListener("abort", () => observer.disconnect());
};
