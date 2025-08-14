import Header from "../mycomponents/Header";
import Hero from "../mycomponents/Hero";
import Footer from "../mycomponents/Footer";
import Benefits from "../mycomponents/Benefits";
import CTA from "../mycomponents/CTA";
import Testimonials from "../mycomponents/Testimonials";


function LandingPage() {
  return (
    <>
      <Header /> {/* 👈 Render the imported page */}
      <Hero />
      <Benefits />
      <CTA />
      <Testimonials />
      <Footer />
    </>
  );
}

export default LandingPage;
