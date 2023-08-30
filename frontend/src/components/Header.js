import '../styles/Header.css'

function Header() {
  return (
    <div className="App-header">
      <form action="/">
        <label for="fname">username: </label>
        <input type="text"></input>
        <label for="lname">password: </label>
        <input type="password"></input>
        <input type="submit" value="Login"></input>
      </form>
    </div>
  )
}

export default Header;

