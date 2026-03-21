import { 
    readFile
} from "node:fs/promises";

export function buildReport(name: string): string {
  return `report:${name}`;
}

export class Reporter {}
