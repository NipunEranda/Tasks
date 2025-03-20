import { createI18n } from "vue-i18n";
import en from "./locale_en.json";

const i18n = createI18n({
    legacy: false,
    locale: "en",
    messages: {
        en,
    },
});

export default i18n;