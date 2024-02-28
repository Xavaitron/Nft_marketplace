import OutlinedCard from "./Components/card";
import ResponsiveAppBar from "./Components/navbar";

export default function App(){
  return <div className="app-wrapper">
    <ResponsiveAppBar></ResponsiveAppBar>
    <OutlinedCard></OutlinedCard>
  </div>
}