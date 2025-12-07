# CryptoWallet Frontend

React-based frontend for the decentralized cryptocurrency wallet system. Built with Vite and vanilla CSS for optimal performance.

## Installation

\`\`\`bash
npm install
\`\`\`

## Development

\`\`\`bash
npm run dev
\`\`\`

Starts development server on http://localhost:5173

## Building

\`\`\`bash
npm run build
\`\`\`

## Project Structure

- `src/App.jsx` - Main application component
- `src/components/` - Reusable components
  - `Navbar.jsx` - Navigation bar
- `src/pages/` - Page components
  - `Login.jsx` - Login page
  - `Register.jsx` - Registration page
  - `Dashboard.jsx` - Wallet dashboard
  - `SendMoney.jsx` - Send funds page
  - `TransactionHistory.jsx` - History page
  - `BlockExplorer.jsx` - Blockchain viewer
- `src/styles/` - CSS stylesheets
  - `App.css` - Global styles
  - `Auth.css` - Authentication pages
  - `Dashboard.css` - Dashboard styles
  - `SendMoney.css` - Send page styles
  - `History.css` - History page styles
  - `BlockExplorer.css` - Explorer styles
- `src/services/` - API integration (ready for services)

## Features

- User authentication with OTP
- Wallet management with UTXO display
- Send money with transaction details
- Transaction history tracking
- Blockchain explorer
- Real-time balance updates
- Responsive design for mobile and desktop

## Environment Variables

Create `.env` file with:

\`\`\`
VITE_API_URL=http://localhost:3001
\`\`\`

## CSS Architecture

All styling uses vanilla CSS with custom properties (CSS variables) for theming:

- `--primary`: Main brand color (#000000)
- `--secondary`: Secondary color (#0066cc)
- `--accent`: Accent color (#00d4ff)
- `--success`: Success color (#00d4a8)
- `--warning`: Warning color (#ffb800)
- `--danger`: Error color (#ff3366)
- `--background`: Background color (#f5f5f5)
- `--surface`: Surface color (#ffffff)
- `--text-primary`: Primary text (#1a1a1a)
- `--text-secondary`: Secondary text (#666666)
- `--border`: Border color (#e0e0e0)
- `--shadow`: Box shadow
- `--shadow-lg`: Large box shadow
