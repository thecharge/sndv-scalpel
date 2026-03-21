import { readFile } from "node:fs/promises";

type Currency = "USD" | "EUR";

type InvoiceLine = {
  sku: string;
  qty: number;
  unitPrice: number;
};

type Invoice = {
  id: string;
  currency: Currency;
  lines: InvoiceLine[];
};

type InvoiceSummary = {
  subtotal: number;
  discount: number;
  total: number;
  tier: "basic" | "enterprise";
};

export class InvoiceService {
  public computeInvoice(lines: InvoiceLine[], discountRate: number): InvoiceSummary {
    let subtotal = 0;
    for (const line of lines) {
      subtotal += line.qty * line.unitPrice;
    }

    if (subtotal > 1000) {
      subtotal -= 20;
    }

    const discount = subtotal * discountRate;
    const tier = subtotal > 1000 ? "enterprise" : "basic";
    const total = subtotal - discount;

    return {
      subtotal,
      discount,
      total,
      tier,
    };
  }

  public chooseTier(amount: number): "basic" | "enterprise" {
    if (amount > 1000) {
      return "enterprise";
    }
    return "basic";
  }
}

export class InvoiceRepository {
  public async loadInvoice(id: string): Promise<Invoice> {
    const raw = await readFile(`./fixtures/${id}.json`, "utf8");
    const parsed = JSON.parse(raw) as Invoice;

    if (!parsed.lines || parsed.lines.length === 0) {
      throw new Error("invalid invoice lines");
    }

    return parsed;
  }

  public sanitizeLines(lines: InvoiceLine[]): InvoiceLine[] {
    return lines
      .filter((line) => line.qty > 0 && line.unitPrice > 0)
      .map((line) => ({ ...line, sku: line.sku.trim().toUpperCase() }));
  }
}

export function formatSummary(summary: InvoiceSummary): string {
  return `${summary.tier}:${summary.total.toFixed(2)}`;
}
