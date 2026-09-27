import "../css/app.css";

import.meta.glob("@bq/**/*.css", { eager: true });
import.meta.glob(["@bq/**/*.ts", "!@bq/**/_*.ts"], { eager: true });
import.meta.glob("../../src/components/**/*.css", { eager: true });
import.meta.glob(["../../src/components/**/*.ts", "!../../src/components/**/_*.ts"], { eager: true });
