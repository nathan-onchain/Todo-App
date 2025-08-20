import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import heroMockup from "@/assets/hero-mockup.webp";

const Hero = () => {
  return (
    <section className="pt-24 pb-16 bg-bg-gradient">
      <div className="container mx-auto px-4">
        <div className="grid lg:grid-cols-2 gap-12 items-center">
          <div className="text-center lg:text-left animate-fade-in">
            <Badge variant="secondary" className="mb-6 px-4 py-2">
              🚀 Trusted by 10,000+ users worldwide
            </Badge>
            
            <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold text-foreground leading-tight mb-6">
              Organize Your Life{" "}
              <span className="bg-text-gradient bg-clip-text text-transparent">
                Effortlessly
              </span>
            </h1>
            
            <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto lg:mx-0">
              Stay on top of your tasks with TaskTonic. Set reminders, organize projects, 
              and achieve your goals — all in one beautifully designed app.
            </p>
            
            <div className="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start mb-8">
              <Button className="shadow-cta hover:shadow-lg transition-all duration-300" asChild>
            <a href="/signup">Get Started Free</a>
          </Button>
              <Button variant="outline" size="lg" className="text-lg px-8 py-3">
                Watch Demo
              </Button>
            </div>
            
            <p className="text-sm text-muted-foreground">
              ✨ No credit card required • 🔒 Your data stays private
            </p>
          </div>
          
          <div className="relative animate-scale-in">
            <div className="relative mx-auto max-w-2xl">
              <img 
                src={heroMockup} 
                alt="TaskTonic app interface on laptop and mobile" 
                className="w-full h-auto rounded-2xl shadow-card"
              />
              <div className="absolute inset-0 bg-gradient-to-t from-primary/5 to-transparent rounded-2xl"></div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Hero;