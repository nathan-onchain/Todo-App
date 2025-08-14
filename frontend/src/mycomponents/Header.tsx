import { Button } from "../components/ui/button";

const Header = () => {
  return (
    <header className="fixed top-0 left-0 right-0 z-50 bg-background/95 backdrop-blur-sm border-b border-border">
      <div className="container mx-auto px-4 h-16 flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
            <span className="text-primary-foreground font-bold text-lg">T</span>
          </div>
          <span className="text-xl font-semibold text-foreground">TaskTonic</span>
        </div>

        <nav className="hidden md:flex items-center space-x-8">
          <a href="#features" className="text-muted-foreground hover:text-foreground transition-colors">
            Features
          </a>
          <a href="#testimonials" className="text-muted-foreground hover:text-foreground transition-colors">
            Reviews
          </a>
          <a href="/privacy" className="text-muted-foreground hover:text-foreground transition-colors">
            Privacy
          </a>
        </nav>

        <div className="flex items-center space-x-3">
          <Button variant="ghost" className="hidden sm:inline-flex" asChild>
            <a href="/login">Login</a>
          </Button>
          <Button className="shadow-cta hover:shadow-lg transition-all duration-300" asChild>
            <a href="/signup">Get Started Free</a>
          </Button>
        </div>
      </div>
    </header>
  );
};

export default Header;
