"use client"

const Navbar = ({ user, onLogout, onNavigate }) => {
  return (
    <nav className="navbar">
      <div className="navbar-container">
        <div className="navbar-brand">
          <h1>CryptoWallet</h1>
        </div>
        <ul className="navbar-menu">
          <li>
            <a onClick={() => onNavigate("dashboard")} className="nav-link">
              Dashboard
            </a>
          </li>
          <li>
            <a onClick={() => onNavigate("send")} className="nav-link">
              Send Money
            </a>
          </li>
          <li>
            <a onClick={() => onNavigate("history")} className="nav-link">
              History
            </a>
          </li>
          <li>
            <a onClick={() => onNavigate("explorer")} className="nav-link">
              Explorer
            </a>
          </li>
        </ul>
        <div className="navbar-user">
          <span className="user-info">{user?.fullName}</span>
          <button onClick={onLogout} className="logout-btn">
            Logout
          </button>
        </div>
      </div>
    </nav>
  )
}

export default Navbar
