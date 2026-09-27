import "@bq/bq_style/reset.css";
import "@bq/bq_style/tokens.css";
import "../css/app.css";

import.meta.glob("@bq/bq_components/src/**/*.css", { eager: true });
import.meta.glob(["@bq/bq_components/src/**/*.ts", "!@bq/bq_components/src/**/_*.ts"], { eager: true });
import.meta.glob("../../src/components/**/*.css", { eager: true });
import.meta.glob(["../../src/components/**/*.ts", "!../../src/components/**/_*.ts"], { eager: true });
