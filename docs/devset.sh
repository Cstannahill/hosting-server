#!/bin/bash

set -e

log_success() {
  echo -e "\033[1;32m[SUCCESS]\033[0m $1"
}

log_error() {
  echo -e "\033[1;31m[ERROR]\033[0m $1"
}

install_or_report() {
  DESC="$1"
  shift
  if "$@"; then
    log_success "$DESC"
  else
    log_error "Failed to install $DESC"
  fi
}

echo "---- Updating package lists ----"
install_or_report "System update" sudo apt update -y

echo "---- Installing Python, pip, and venv ----"
install_or_report "Python 3, pip, venv" sudo apt install python3 python3-pip python3-venv -y

echo "---- Installing Python dev tools ----"
install_or_report "Python3-dev, build-essential" sudo apt install python3-dev build-essential -y

echo "---- Installing Poetry (Python package manager) ----"
install_or_report "Poetry" pip3 install poetry

echo "---- Installing Node Version Manager (nvm) ----"
if [ ! -d "$HOME/.nvm" ]; then
  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
  export NVM_DIR="$HOME/.nvm"
  [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
  log_success "nvm installed"
else
  export NVM_DIR="$HOME/.nvm"
  [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
  log_success "nvm already present"
fi

echo "---- Installing latest LTS Node.js and npm ----"
nvm install --lts && nvm use --lts && log_success "Node.js and npm installed via nvm" || log_error "Node.js install failed"

echo "---- Installing global TypeScript ----"
install_or_report "TypeScript (npm global)" npm install -g typescript

echo "---- Installing pnpm (npm global)" 
install_or_report "pnpm" npm install -g pnpm

echo "---- Installing JS/TS linters ----"
install_or_report "ESLint (npm global)" npm install -g eslint
install_or_report "Prettier (npm global)" npm install -g prettier

echo "---- Installing Python linters ----"
install_or_report "black (pip3 global)" pip3 install black
install_or_report "flake8 (pip3 global)" pip3 install flake8
install_or_report "isort (pip3 global)" pip3 install isort

echo "---- Installing CLI dev tools: httpie, jq ----"
install_or_report "httpie" sudo apt install httpie -y
install_or_report "jq" sudo apt install jq -y

echo "---- Installing Database clients ----"
install_or_report "PostgreSQL client" sudo apt install postgresql-client -y
install_or_report "MySQL client" sudo apt install mysql-client -y
install_or_report "MongoDB tools" sudo apt install mongodb-clients -y
install_or_report "SQLite3" sudo apt install sqlite3 -y

# SQL Server client (msodbcsql) is a bit more involved
echo "---- Installing SQL Server ODBC tools ----"
curl https://packages.microsoft.com/keys/microsoft.asc | sudo apt-key add - \
  && curl https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/prod.list | sudo tee /etc/apt/sources.list.d/mssql-release.list \
  && sudo apt update -y \
  && sudo ACCEPT_EULA=Y apt install -y msodbcsql18 \
  && log_success "SQL Server ODBC driver installed" \
  || log_error "Failed to install SQL Server ODBC driver"

echo "---- All requested development tools installed! ----"
