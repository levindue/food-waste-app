import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import '../styles/Header.css';

function Header() {
  const location = useLocation();
  const targetPath = location.pathname === '/' ? '/list' : '/';

  return (
    <div className="App-header">
      <form>
        <label htmlFor="fname">username: </label>
        <input type="text" />
        <label htmlFor="lname">password: </label>
        <input type="password" />
        <input type="submit" value="Login" />
      </form>

      <Link to={targetPath}>
        <button>{location.pathname === '/' ? 'Go to List Page' : 'Go to Home Page'}</button>
      </Link>
    </div>
  );
}

export default Header;
