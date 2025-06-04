import lazy_loader as lazy

__version__ = "1.12.0"
__getattr__, __dir__, __all__ = lazy.attach_stub(__name__, __file__)
