
async function get_data() {
  try {
    await fetch("http://127.0.0.1:8080/api/list_people", {
      method: "GET",
      mode: "cors",
    }).then(data => { return JSON.parse(data) });
  }
  catch (error) {
    console.log(error)
  }
}

function Test() {
  let data = get_data();

  return (
    <div>
      <h1>{data[0]}</h1>
    </div>
  )
}

export default Test;
