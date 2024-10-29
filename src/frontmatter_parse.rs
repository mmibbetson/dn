// if its yaml, first line is "^---$"
const YAML_TITLE_PATTERN = r"(?m)^\+title:\s*(.*?)\n?";
const YAML_DATE_PATTERN = r"(?m)^\+date:\s*(.*?)\n?";
const YAML_FILETAGS_PATTERN = r"(?m)^\+filetags:\s*(.*?)\n?";
const YAML_IDENTIFIER_PATTERN = r"(?m)^\+identifier:\s*(.*?)\n?";

// if its toml, first line is "^+++$"
const TOML_TITLE_PATTERN = r"(?m)^title:\s*(\".*\")\n?";
const TOML_DATE_PATTERN = 
const TOML_FILETAGS_PATTERN = 
const TOML_IDENTIFIER_PATTERN = 

// if its org, first line is "^\#\+.*"
const ORG_TITLE_PATTERN = r"(?m)^\#\+title:\s*(.*)\n?";
const ORG_DATE_PATTERN = r"(?m)^\#\+date:\s*(.*)\n?";
const ORG_FILETAGS_PATTERN = r"(?m)^\#\+filetags:\s*(.*)\n?";
const ORG_IDENTIFIER_PATTERN = r"(?m)^\#\+identifier:\s*(.*)\n?";

// if its yaml, first line is "^[title|date|filetags|identifier]:\s*"
const TEXT_TITLE_PATTERN = r"(?m)^\+title:\s*(.*?)\n?";
const TEXT_DATE_PATTERN = r"(?m)^\+date:\s*(.*?)\n?";
const TEXT_FILETAGS_PATTERN = r"(?m)^\+filetags:\s*(.*?)\n?";
const TEXT_IDENTIFIER_PATTERN = r"(?m)^\+identifier:\s*(.*?)\n?";

