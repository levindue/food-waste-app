import React, { useState, useEffect } from 'react';
import { Link, useLocation } from 'react-router-dom';
import '../styles/Header.css';
import logo from '../images/logo.png'

function Header() {

  const [loggedIn, setLoggedIn] = useState(false);
  const [username, setUsername] = useState('');

  useEffect(() => {
    const storedUsername = localStorage.getItem('username');
    if (storedUsername) {
      setLoggedIn(true);
      setUsername(storedUsername);
    }
  }, []);

  const handleLogin = (e) => {
    e.preventDefault();

    const enteredUsername = e.target.username.value;

    setLoggedIn(true);
    setUsername(enteredUsername);

    localStorage.setItem('username', enteredUsername);
  };

  const handleLogout = () => {
    setLoggedIn(false);
    setUsername('');

    localStorage.removeItem('username');
  };

  const location = useLocation();
  const target = location.pathname === '/' ? '/dashboard' : '/';

  return (
    <div className="App-header">
      <div className="logo-container">
        <img src={logo} alt="Logo" className="logo" />
      </div>
      {loggedIn ? (
        <div className='App-welcome'>
    <h1 className='Welcome-text'>Welcome {username}!</h1>
          <div className="dashboard-logout">
            <Link to={target}>
              <button>{location.pathname === '/' ? 'Dashboard' : 'Home'}</button>
            </Link>
            <button onClick={handleLogout}>Logout</button>
          </div>
        </div>
      ) : (
        <form onSubmit={handleLogin}>
          <label htmlFor="username">Username: </label>
          <input autoComplete='off' type="text" name="username" />
          <input type="submit" value="Login" />
        </form>
      )}
    </div>
  );
}

export default Header;
